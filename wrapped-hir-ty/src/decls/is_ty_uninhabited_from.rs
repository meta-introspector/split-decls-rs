macro_rules! deps {
    () => {
        TraitEnvironment!();
        UninhabitedFrom!();
    };
}

macro_rules! is_ty_uninhabited_from {
    () => {
        deps!();
        # [doc = " Checks whether a type is visibly uninhabited from a particular module."] pub (crate) fn is_ty_uninhabited_from < 'db > (infcx : & InferCtxt < 'db > , ty : Ty < 'db > , target_mod : ModuleId , env : Arc < TraitEnvironment < 'db > > ,) -> bool { let _p = tracing :: info_span ! ("is_ty_uninhabited_from" , ? ty) . entered () ; let mut uninhabited_from = UninhabitedFrom :: new (infcx , target_mod , env) ; let inhabitedness = ty . visit_with (& mut uninhabited_from) ; inhabitedness == BREAK_VISIBLY_UNINHABITED }
    };
}

is_ty_uninhabited_from!();