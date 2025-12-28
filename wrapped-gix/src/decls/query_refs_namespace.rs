macro_rules! deps {
    () => {
        Core!();
        Error!();
    };
}

macro_rules! query_refs_namespace {
    () => {
        deps!();
        pub (crate) fn query_refs_namespace (config : & gix_config :: File < 'static > , lenient_config : bool ,) -> Result < Option < gix_ref :: Namespace > , config :: refs_namespace :: Error > { let key = "gitoxide.core.refsNamespace" ; config . string (key) . map (| ns | gitoxide :: Core :: REFS_NAMESPACE . try_into_refs_namespace (ns)) . transpose () . with_leniency (lenient_config) }
    };
}

query_refs_namespace!();