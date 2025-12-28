macro_rules! deps {
    () => {
        Core!();
        ObjectKindHint!();
    };
}

macro_rules! disambiguate_hint {
    () => {
        deps!();
        # [cfg (feature = "revision")] pub (crate) fn disambiguate_hint (config : & gix_config :: File < 'static > , lenient_config : bool ,) -> Result < Option < crate :: revision :: spec :: parse :: ObjectKindHint > , config :: key :: GenericErrorWithValue > { match config . string ("core.disambiguate") { None => Ok (None) , Some (value) => Core :: DISAMBIGUATE . try_into_object_kind_hint (value) . with_leniency (lenient_config) , } }
    };
}

disambiguate_hint!();