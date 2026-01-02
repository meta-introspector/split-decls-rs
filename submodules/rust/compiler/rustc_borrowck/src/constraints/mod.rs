mkuse!{use std :: fmt ;}
mkuse!{use std :: ops :: Index ;}
mkuse!{use rustc_index :: { IndexSlice , IndexVec } ;}
mkuse!{use rustc_middle :: mir :: ConstraintCategory ;}
mkuse!{use rustc_middle :: ty :: { RegionVid , TyCtxt , VarianceDiagInfo } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: type_check :: Locations ;}
mkmod!{graph, { 
                getname!(graph);
                getsrc!(graph);
                getpath!(graph);
                get_deps!(graph);
                get_crates!(graph);
                mkinclude!(graph);
                 
            }}
mkitem!{mkstruct!{# [doc = " A set of NLL region constraints. These include \"outlives\""] # [doc = " constraints of the form `R1: R2`. Each constraint is identified by"] # [doc = " a unique `OutlivesConstraintIndex` and you can index into the set"] # [doc = " (`constraint_set[i]`) to access the constraint details."] # [derive (Clone , Debug , Default)] pub (crate) struct OutlivesConstraintSet < 'tcx > { outlives : IndexVec < OutlivesConstraintIndex , OutlivesConstraint < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx > OutlivesConstraintSet < 'tcx > { pub (crate) fn push (& mut self , constraint : OutlivesConstraint < 'tcx >) { debug ! ("OutlivesConstraintSet::push({:?})" , constraint) ; if constraint . sup == constraint . sub { return ; } self . outlives . push (constraint) ; } # [doc = " Constructs a \"normal\" graph from the constraint set; the graph makes it"] # [doc = " easy to find the constraints affecting a particular region."] # [doc = ""] # [doc = " N.B., this graph contains a \"frozen\" view of the current"] # [doc = " constraints. Any new constraints added to the `OutlivesConstraintSet`"] # [doc = " after the graph is built will not be present in the graph."] pub (crate) fn graph (& self , num_region_vars : usize) -> graph :: NormalConstraintGraph { graph :: ConstraintGraph :: new (graph :: Normal , self , num_region_vars) } # [doc = " Like `graph`, but constraints a reverse graph where `R1: R2`"] # [doc = " represents an edge `R2 -> R1`."] pub (crate) fn reverse_graph (& self , num_region_vars : usize) -> graph :: ReverseConstraintGraph { graph :: ConstraintGraph :: new (graph :: Reverse , self , num_region_vars) } pub (crate) fn outlives (& self ,) -> & IndexSlice < OutlivesConstraintIndex , OutlivesConstraint < 'tcx > > { & self . outlives } }}}
mkitem!{mkimpl!{impl < 'tcx > Index < OutlivesConstraintIndex > for OutlivesConstraintSet < 'tcx > { type Output = OutlivesConstraint < 'tcx > ; fn index (& self , i : OutlivesConstraintIndex) -> & Self :: Output { & self . outlives [i] } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , PartialEq , Eq)] pub struct OutlivesConstraint < 'tcx > { # [doc = " The region SUP must outlive SUB..."] pub sup : RegionVid , # [doc = " Region that must be outlived."] pub sub : RegionVid , # [doc = " Where did this constraint arise?"] pub locations : Locations , # [doc = " The `Span` associated with the creation of this constraint."] # [doc = " This should be used in preference to obtaining the span from"] # [doc = " `locations`, since the `locations` may give a poor span"] # [doc = " in some cases (e.g. converting a constraint from a promoted)."] pub span : Span , # [doc = " What caused this constraint?"] pub category : ConstraintCategory < 'tcx > , # [doc = " Variance diagnostic information"] pub variance_info : VarianceDiagInfo < TyCtxt < 'tcx > > , # [doc = " If this constraint is promoted from closure requirements."] pub from_closure : bool , }}}
mkitem!{mkimpl!{impl < 'tcx > fmt :: Debug for OutlivesConstraint < 'tcx > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "({:?}: {:?}) due to {:?} ({:?}) ({:?})" , self . sup , self . sub , self . locations , self . variance_info , self . category ,) } }}}
mkitem!{rustc_index :: newtype_index ! { # [debug_format = "OutlivesConstraintIndex({})"] pub (crate) struct OutlivesConstraintIndex { } }}
mkitem!{rustc_index :: newtype_index ! { # [orderable] # [debug_format = "ConstraintSccIndex({})"] pub struct ConstraintSccIndex { } }}