macro_rules! deps {
    () => {
        Core!();
        Error!();
    };
}

macro_rules! query_refupdates {
    () => {
        deps!();
        pub (crate) fn query_refupdates (config : & gix_config :: File < 'static > , lenient_config : bool ,) -> Result < Option < gix_ref :: store :: WriteReflog > , Error > { let key = "core.logAllRefUpdates" ; Core :: LOG_ALL_REF_UPDATES . try_into_ref_updates (config . boolean (key)) . with_leniency (lenient_config) . map_err (Into :: into) }
    };
}

query_refupdates!();