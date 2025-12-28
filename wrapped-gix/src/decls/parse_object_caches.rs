macro_rules! deps {
    () => {
        Error!();
        Core!();
    };
}

macro_rules! parse_object_caches {
    () => {
        deps!();
        # [doc = " Return `(pack_cache_bytes, object_cache_bytes)` as parsed from gix-config"] pub (crate) fn parse_object_caches (config : & gix_config :: File < 'static > , lenient : bool , mut filter_config_section : fn (& gix_config :: file :: Metadata) -> bool ,) -> Result < (Option < usize > , Option < usize > , usize) , Error > { let static_pack_cache_limit = config . integer_filter ("gitoxide.core.deltaBaseCacheLimit" , & mut filter_config_section) . map (| res | gitoxide :: Core :: DEFAULT_PACK_CACHE_MEMORY_LIMIT . try_into_usize (res)) . transpose () . with_leniency (lenient) ? ; let pack_cache_bytes = config . integer_filter ("core.deltaBaseCacheLimit" , & mut filter_config_section) . map (| res | Core :: DELTA_BASE_CACHE_LIMIT . try_into_usize (res)) . transpose () . with_leniency (lenient) ? ; let object_cache_bytes = config . integer_filter ("gitoxide.objects.cacheLimit" , & mut filter_config_section) . map (| res | gitoxide :: Objects :: CACHE_LIMIT . try_into_usize (res)) . transpose () . with_leniency (lenient) ? . unwrap_or_default () ; Ok ((static_pack_cache_limit , pack_cache_bytes , object_cache_bytes)) }
    };
}

parse_object_caches!();