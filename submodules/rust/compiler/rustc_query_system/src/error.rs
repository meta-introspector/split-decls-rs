mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_hir :: limit :: Limit ;}
mkuse!{use rustc_macros :: { Diagnostic , Subdiagnostic } ;}
mkuse!{use rustc_span :: { Span , Symbol } ;}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (query_system_cycle_stack_middle)] pub (crate) struct CycleStack { # [primary_span] pub span : Span , pub desc : String , }}}
mkitem!{mkenum!{# [derive (Copy , Clone)] pub enum HandleCycleError { Error , Fatal , DelayBug , Stash , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum StackCount { # [note (query_system_cycle_stack_single)] Single , # [note (query_system_cycle_stack_multiple)] Multiple , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum Alias { # [note (query_system_cycle_recursive_ty_alias)] # [help (query_system_cycle_recursive_ty_alias_help1)] # [help (query_system_cycle_recursive_ty_alias_help2)] Ty , # [note (query_system_cycle_recursive_trait_alias)] Trait , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (query_system_cycle_usage)] pub (crate) struct CycleUsage { # [primary_span] pub span : Span , pub usage : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (query_system_cycle , code = E0391)] pub (crate) struct Cycle { # [primary_span] pub span : Span , pub stack_bottom : String , # [subdiagnostic] pub cycle_stack : Vec < CycleStack > , # [subdiagnostic] pub stack_count : StackCount , # [subdiagnostic] pub alias : Option < Alias > , # [subdiagnostic] pub cycle_usage : Option < CycleUsage > , # [note] pub note_span : () , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (query_system_reentrant)] pub (crate) struct Reentrant ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (query_system_increment_compilation)] # [help] # [note (query_system_increment_compilation_note1)] # [note (query_system_increment_compilation_note2)] pub (crate) struct IncrementCompilation { pub run_cmd : String , pub dep_node : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [help] # [diag (query_system_query_overflow)] pub struct QueryOverflow { # [primary_span] pub span : Span , # [subdiagnostic] pub note : QueryOverflowNote , pub suggested_limit : Limit , pub crate_name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (query_system_overflow_note)] pub struct QueryOverflowNote { pub desc : String , pub depth : usize , }}}