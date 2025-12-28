macro_rules! deps {
    () => {
        String!();
        SchemePermission!();
        Attributes!();
        Algorithm!();
        Config!();
        Personas!();
        ObjectKindHint!();
        Kind!();
        Environment!();
        Clone!();
        Rewrite!();
    };
}

macro_rules! Cache {
    () => {
        deps!();
        # [doc = " Utility type to keep pre-obtained configuration values, only for those required during initial setup"] # [doc = " and other basic operations that are common enough to warrant a permanent cache."] # [doc = ""] # [doc = " All other values are obtained lazily using `OnceCell`."] # [derive (Clone)] pub (crate) struct Cache { pub resolved : crate :: Config , # [doc = " The hex-length to assume when shortening object ids. If `None`, it should be computed based on the approximate object count."] pub hex_len : Option < usize > , # [doc = " true if the repository is designated as 'bare', without work tree."] pub is_bare : bool , # [doc = " The type of hash to use."] pub object_hash : gix_hash :: Kind , # [doc = " If true, multi-pack indices, whether present or not, may be used by the object database."] pub use_multi_pack_index : bool , # [doc = " The representation of `core.logallrefupdates`, or `None` if the variable wasn't set."] pub reflog : Option < gix_ref :: store :: WriteReflog > , # [doc = " The representation of `gitoxide.core.refsNamespace`, or `None` if the variable wasn't set."] pub refs_namespace : Option < gix_ref :: Namespace > , # [doc = " The configured user agent for presentation to servers."] pub (crate) user_agent : OnceCell < String > , # [doc = " identities for later use, lazy initialization."] pub (crate) personas : OnceCell < identity :: Personas > , # [doc = " A lazily loaded rewrite list for remote urls"] pub (crate) url_rewrite : OnceCell < crate :: remote :: url :: Rewrite > , # [doc = " The lazy-loaded rename information for diffs."] # [cfg (feature = "blob-diff")] pub (crate) diff_renames : OnceCell < (Option < crate :: diff :: Rewrites > , bool) > , # [doc = " A lazily loaded mapping to know which url schemes to allow"] # [cfg (any (feature = "blocking-network-client" , feature = "async-network-client"))] pub (crate) url_scheme : OnceCell < crate :: remote :: url :: SchemePermission > , # [doc = " The algorithm to use when diffing blobs"] # [cfg (feature = "blob-diff")] pub (crate) diff_algorithm : OnceCell < gix_diff :: blob :: Algorithm > , # [doc = " The amount of bytes to use for a memory backed delta pack cache. If `Some(0)`, no cache is used, if `None`"] # [doc = " a standard cache is used which costs near to nothing and always pays for itself."] pub (crate) pack_cache_bytes : Option < usize > , # [doc = " The amount of bytes to use for caching whole objects, or 0 to turn it off entirely."] pub (crate) object_cache_bytes : usize , # [doc = " The amount of bytes we can hold in our static LRU cache. Otherwise, go with the defaults."] pub (crate) static_pack_cache_limit_bytes : Option < usize > , # [doc = " The config section filter from the options used to initialize this instance. Keep these in sync!"] filter_config_section : fn (& gix_config :: file :: Metadata) -> bool , # [doc = " The object kind to pick if a prefix is ambiguous."] # [cfg (feature = "revision")] pub object_kind_hint : Option < crate :: revision :: spec :: parse :: ObjectKindHint > , # [doc = " If true, we are on a case-insensitive file system."] pub ignore_case : bool , # [doc = " If true, we should default what's possible if something is misconfigured, on case by case basis, to be more resilient."] # [doc = " Also, available in options! Keep in sync!"] pub lenient_config : bool , # [cfg_attr (not (feature = "worktree-mutation") , allow (dead_code))] attributes : crate :: open :: permissions :: Attributes , environment : crate :: open :: permissions :: Environment , }
    };
}

Cache!()