mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { Instance , TyCtxt } ;}
mkmod!{abi_check, { 
                getname!(abi_check);
                getsrc!(abi_check);
                getpath!(abi_check);
                get_deps!(abi_check);
                get_crates!(abi_check);
                mkinclude!(abi_check);
                 
            }}
mkmod!{move_check, { 
                getname!(move_check);
                getsrc!(move_check);
                getpath!(move_check);
                get_deps!(move_check);
                get_crates!(move_check);
                mkinclude!(move_check);
                 
            }}

macro_rules! check_mono_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_mono_item in module {}", module_path!());
    };
}

mkfn!{
    check_mono_item_introspect!();
    fn check_mono_item < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx >) { let body = tcx . instance_mir (instance . def) ; abi_check :: check_feature_dependent_abi (tcx , instance , body) ; move_check :: check_moves (tcx , instance , body) ; }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (super) fn provide (providers : & mut Providers) { * providers = Providers { check_mono_item , skip_move_check_fns : move_check :: skip_move_check_fns , .. * providers } }
}