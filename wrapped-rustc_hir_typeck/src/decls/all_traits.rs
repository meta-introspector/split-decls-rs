macro_rules! deps {
    () => {
        TraitInfo!();
    };
}

macro_rules! all_traits {
    () => {
        deps!();
        # [doc = " Retrieves all traits in this crate and any dependent crates,"] # [doc = " and wraps them into `TraitInfo` for custom sorting."] pub (crate) fn all_traits (tcx : TyCtxt < '_ >) -> Vec < TraitInfo > { tcx . all_traits_including_private () . map (| def_id | TraitInfo { def_id }) . collect () }
    };
}

all_traits!();