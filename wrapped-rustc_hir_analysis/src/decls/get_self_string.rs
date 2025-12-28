macro_rules! get_self_string {
    () => {
        # [doc = " Gets the string for an explicit self declaration, e.g. \"self\", \"&self\","] # [doc = " etc."] fn get_self_string < 'tcx , P > (self_arg_ty : Ty < 'tcx > , is_self_ty : P) -> String where P : Fn (Ty < 'tcx >) -> bool , { if is_self_ty (self_arg_ty) { "self" . to_owned () } else if let ty :: Ref (_ , ty , mutbl) = self_arg_ty . kind () && is_self_ty (* ty) { match mutbl { hir :: Mutability :: Not => "&self" . to_owned () , hir :: Mutability :: Mut => "&mut self" . to_owned () , } } else { format ! ("self: {self_arg_ty}") } }
    };
}

get_self_string!()