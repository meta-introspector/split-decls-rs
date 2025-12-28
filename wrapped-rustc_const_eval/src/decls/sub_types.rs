macro_rules! sub_types {
    () => {
        # [doc = " Returns whether `src` is a subtype of `dest`, i.e. `src <: dest`."] pub fn sub_types < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx > , src : Ty < 'tcx > , dest : Ty < 'tcx > ,) -> bool { relate_types (tcx , typing_env , Variance :: Covariant , src , dest) }
    };
}

sub_types!();