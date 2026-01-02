mkuse!{use std :: borrow :: Cow ;}
mkuse!{use rustc_target :: callconv :: PassMode ;}
mkuse!{use crate :: prelude :: * ;}

macro_rules! add_args_header_comment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_args_header_comment in module {}", module_path!());
    };
}

mkfn!{
    add_args_header_comment_introspect!();
    pub (super) fn add_args_header_comment (fx : & mut FunctionCx < '_ , '_ , '_ >) { if fx . clif_comments . enabled () { fx . add_global_comment ("kind  loc.idx   param    pass mode                            ty" . to_string () ,) ; } }
}

macro_rules! add_arg_comment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_arg_comment in module {}", module_path!());
    };
}

mkfn!{
    add_arg_comment_introspect!();
    pub (super) fn add_arg_comment < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , kind : & str , local : Option < mir :: Local > , local_field : Option < usize > , params : & [Value] , arg_abi_mode : & PassMode , arg_layout : TyAndLayout < 'tcx > ,) { if ! fx . clif_comments . enabled () { return ; } let local = if let Some (local) = local { Cow :: Owned (format ! ("{:?}" , local)) } else { Cow :: Borrowed ("???") } ; let local_field = if let Some (local_field) = local_field { Cow :: Owned (format ! (".{}" , local_field)) } else { Cow :: Borrowed ("") } ; let params = match params { [] => Cow :: Borrowed ("-") , [param] => Cow :: Owned (format ! ("= {:?}" , param)) , [param_a , param_b] => Cow :: Owned (format ! ("= {:?},{:?}" , param_a , param_b)) , params => Cow :: Owned (format ! ("= {}" , params . iter () . map (ToString :: to_string) . collect ::< Vec < _ >> () . join (","))) , } ; let pass_mode = format ! ("{:?}" , arg_abi_mode) ; fx . add_global_comment (format ! ("{kind:5}{local:>3}{local_field:<5} {params:10} {pass_mode:36} {ty:?}" , kind = kind , local = local , local_field = local_field , params = params , pass_mode = pass_mode , ty = arg_layout . ty ,)) ; }
}

macro_rules! add_locals_header_comment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_locals_header_comment in module {}", module_path!());
    };
}

mkfn!{
    add_locals_header_comment_introspect!();
    pub (super) fn add_locals_header_comment (fx : & mut FunctionCx < '_ , '_ , '_ >) { if fx . clif_comments . enabled () { fx . add_global_comment (String :: new ()) ; fx . add_global_comment ("kind  local ty                              size align (abi)" . to_string () ,) ; } }
}

macro_rules! add_local_place_comments_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_local_place_comments in module {}", module_path!());
    };
}

mkfn!{
    add_local_place_comments_introspect!();
    pub (super) fn add_local_place_comments < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , place : CPlace < 'tcx > , local : Local ,) { if ! fx . clif_comments . enabled () { return ; } let TyAndLayout { ty , layout } = place . layout () ; let rustc_abi :: LayoutData { size , align , .. } = layout . 0 . 0 ; let (kind , extra) = place . debug_comment () ; fx . add_global_comment (format ! ("{:<5} {:5} {:30} {:4}b {}{}{}" , kind , format ! ("{:?}" , local) , format ! ("{:?}" , ty) , size . bytes () , align . abi . bytes () , if extra . is_empty () { "" } else { "                " } , extra ,)) ; }
}