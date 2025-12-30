// Generated macro for add_arg_comment (function)
macro_rules! Depcrate_abi_commentsadd_arg_comment {
() => {
// Module: crate::abi::comments
// Provides: {"add_arg_comment"}
// Dependencies: {}
pub (super) fn add_arg_comment < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , kind : & str , local : Option < mir :: Local > , local_field : Option < usize > , params : & [Value] , arg_abi_mode : & PassMode , arg_layout : TyAndLayout < 'tcx > ,) { if ! fx . clif_comments . enabled () { return ; } let local = if let Some (local) = local { Cow :: Owned (format ! ("{:?}" , local)) } else { Cow :: Borrowed ("???") } ; let local_field = if let Some (local_field) = local_field { Cow :: Owned (format ! (".{}" , local_field)) } else { Cow :: Borrowed ("") } ; let params = match params { [] => Cow :: Borrowed ("-") , [param] => Cow :: Owned (format ! ("= {:?}" , param)) , [param_a , param_b] => Cow :: Owned (format ! ("= {:?},{:?}" , param_a , param_b)) , params => Cow :: Owned (format ! ("= {}" , params . iter () . map (ToString :: to_string) . collect ::< Vec < _ >> () . join (","))) , } ; let pass_mode = format ! ("{:?}" , arg_abi_mode) ; fx . add_global_comment (format ! ("{kind:5}{local:>3}{local_field:<5} {params:10} {pass_mode:36} {ty:?}" , kind = kind , local = local , local_field = local_field , params = params , pass_mode = pass_mode , ty = arg_layout . ty ,)) ; }
};
}
