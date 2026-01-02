mkuse!{use std :: cmp :: Ordering ;}
mkuse!{use either :: Either ;}
mkuse!{use itertools :: Itertools ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap } ;}
mkuse!{use rustc_data_structures :: graph :: DirectedGraph ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_middle :: mir :: coverage :: { CounterId , CovTerm , Expression , ExpressionId , Op } ;}
mkuse!{use crate :: coverage :: counters :: balanced_flow :: BalancedFlowGraph ;}
mkuse!{use crate :: coverage :: counters :: node_flow :: { CounterTerm , NodeCounters , NodeFlowData , node_flow_data_for_balanced_graph , } ;}
mkuse!{use crate :: coverage :: graph :: { BasicCoverageBlock , CoverageGraph } ;}
mkmod!{balanced_flow, { 
                getname!(balanced_flow);
                getsrc!(balanced_flow);
                getpath!(balanced_flow);
                get_deps!(balanced_flow);
                get_crates!(balanced_flow);
                mkinclude!(balanced_flow);
                 
            }}
mkmod!{node_flow, { 
                getname!(node_flow);
                getsrc!(node_flow);
                getpath!(node_flow);
                get_deps!(node_flow);
                get_crates!(node_flow);
                mkinclude!(node_flow);
                 
            }}
mkitem!{mkstruct!{# [doc = " Struct containing the results of [`prepare_bcb_counters_data`]."] pub (crate) struct BcbCountersData { pub (crate) node_flow_data : NodeFlowData < BasicCoverageBlock > , pub (crate) priority_list : Vec < BasicCoverageBlock > , }}}

macro_rules! prepare_bcb_counters_data_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_bcb_counters_data in module {}", module_path!());
    };
}

mkfn!{
    prepare_bcb_counters_data_introspect!();
    # [doc = " Analyzes the coverage graph to create intermediate data structures that"] # [doc = " will later be used (during codegen) to create physical counters or counter"] # [doc = " expressions for each BCB node that needs one."] pub (crate) fn prepare_bcb_counters_data (graph : & CoverageGraph) -> BcbCountersData { let balanced_graph = BalancedFlowGraph :: for_graph (graph , | n | ! graph [n] . is_out_summable) ; let node_flow_data = node_flow_data_for_balanced_graph (& balanced_graph) ; let priority_list = make_node_flow_priority_list (graph , balanced_graph) ; BcbCountersData { node_flow_data , priority_list } }
}

macro_rules! make_node_flow_priority_list_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_node_flow_priority_list in module {}", module_path!());
    };
}

mkfn!{
    make_node_flow_priority_list_introspect!();
    # [doc = " Arranges the nodes in `balanced_graph` into a list, such that earlier nodes"] # [doc = " take priority in being given a counter expression instead of a physical counter."] fn make_node_flow_priority_list (graph : & CoverageGraph , balanced_graph : BalancedFlowGraph < & CoverageGraph > ,) -> Vec < BasicCoverageBlock > { let is_reloop_node = IndexVec :: < BasicCoverageBlock , _ > :: from_fn_n (| node | match graph . successors [node] . as_slice () { & [succ] => graph . dominates (succ , node) , _ => false , } , graph . num_nodes () ,) ; let mut nodes = balanced_graph . iter_nodes () . rev () . collect :: < Vec < _ > > () ; assert_eq ! (nodes [0] , balanced_graph . sink) ; nodes [1 ..] . sort_by (| & a , & b | { Ordering :: Equal . then_with (| | Ord :: cmp (& graph [a] . is_out_summable , & graph [b] . is_out_summable)) . then_with (| | Ord :: cmp (& is_reloop_node [a] , & is_reloop_node [b]) . reverse ()) . then_with (| | graph . cmp_in_dominator_order (a , b) . reverse ()) }) ; nodes }
}

macro_rules! transcribe_counters_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function transcribe_counters in module {}", module_path!());
    };
}

mkfn!{
    transcribe_counters_introspect!();
    pub (crate) fn transcribe_counters (old : & NodeCounters < BasicCoverageBlock > , bcb_needs_counter : & DenseBitSet < BasicCoverageBlock > , bcbs_seen : & DenseBitSet < BasicCoverageBlock > ,) -> CoverageCounters { let mut new = CoverageCounters :: with_num_bcbs (bcb_needs_counter . domain_size ()) ; for bcb in bcb_needs_counter . iter () { if ! bcbs_seen . contains (bcb) { new . set_node_counter (bcb , CovTerm :: Zero) ; continue ; } let (mut pos , mut neg) : (Vec < _ > , Vec < _ >) = old . counter_terms [bcb] . iter () . filter (| term | bcbs_seen . contains (term . node)) . partition_map (| & CounterTerm { node , op } | match op { Op :: Add => Either :: Left (node) , Op :: Subtract => Either :: Right (node) , }) ; pos . sort () ; neg . sort () ; let mut new_counters_for_sites = | sites : Vec < BasicCoverageBlock > | { sites . into_iter () . map (| node | new . ensure_phys_counter (node)) . collect :: < Vec < _ > > () } ; let pos = new_counters_for_sites (pos) ; let neg = new_counters_for_sites (neg) ; let pos_counter = new . make_sum (& pos) . unwrap_or (CovTerm :: Zero) ; let new_counter = new . make_subtracted_sum (pos_counter , & neg) ; new . set_node_counter (bcb , new_counter) ; } new }
}
mkitem!{mkstruct!{# [doc = " Generates and stores coverage counter and coverage expression information"] # [doc = " associated with nodes in the coverage graph."] pub (super) struct CoverageCounters { # [doc = " List of places where a counter-increment statement should be injected"] # [doc = " into MIR, each with its corresponding counter ID."] pub (crate) phys_counter_for_node : FxIndexMap < BasicCoverageBlock , CounterId > , pub (crate) next_counter_id : CounterId , # [doc = " Coverage counters/expressions that are associated with individual BCBs."] pub (crate) node_counters : IndexVec < BasicCoverageBlock , Option < CovTerm > > , # [doc = " Table of expression data, associating each expression ID with its"] # [doc = " corresponding operator (+ or -) and its LHS/RHS operands."] pub (crate) expressions : IndexVec < ExpressionId , Expression > , # [doc = " Remember expressions that have already been created (or simplified),"] # [doc = " so that we don't create unnecessary duplicates."] expressions_memo : FxHashMap < Expression , CovTerm > , }}}
mkitem!{mkimpl!{impl CoverageCounters { fn with_num_bcbs (num_bcbs : usize) -> Self { Self { phys_counter_for_node : FxIndexMap :: default () , next_counter_id : CounterId :: ZERO , node_counters : IndexVec :: from_elem_n (None , num_bcbs) , expressions : IndexVec :: new () , expressions_memo : FxHashMap :: default () , } } # [doc = " Returns the physical counter for the given node, creating it if necessary."] fn ensure_phys_counter (& mut self , bcb : BasicCoverageBlock) -> CovTerm { let id = * self . phys_counter_for_node . entry (bcb) . or_insert_with (| | { let id = self . next_counter_id ; self . next_counter_id = id + 1 ; id }) ; CovTerm :: Counter (id) } fn make_expression (& mut self , lhs : CovTerm , op : Op , rhs : CovTerm) -> CovTerm { let new_expr = Expression { lhs , op , rhs } ; * self . expressions_memo . entry (new_expr . clone ()) . or_insert_with (| | { let id = self . expressions . push (new_expr) ; CovTerm :: Expression (id) }) } # [doc = " Creates a counter that is the sum of the given counters."] # [doc = ""] # [doc = " Returns `None` if the given list of counters was empty."] fn make_sum (& mut self , counters : & [CovTerm]) -> Option < CovTerm > { counters . iter () . copied () . reduce (| accum , counter | self . make_expression (accum , Op :: Add , counter)) } # [doc = " Creates a counter whose value is `lhs - SUM(rhs)`."] fn make_subtracted_sum (& mut self , lhs : CovTerm , rhs : & [CovTerm]) -> CovTerm { let Some (rhs_sum) = self . make_sum (rhs) else { return lhs } ; self . make_expression (lhs , Op :: Subtract , rhs_sum) } fn set_node_counter (& mut self , bcb : BasicCoverageBlock , counter : CovTerm) -> CovTerm { let existing = self . node_counters [bcb] . replace (counter) ; assert ! (existing . is_none () , "node {bcb:?} already has a counter: {existing:?} => {counter:?}") ; counter } }}}