// Generated macro for CACHE (static)
macro_rules! Depcrate_tz_systemCACHE {
() => {
// Module: crate::tz::system
// Provides: {"CACHE"}
// Dependencies: {}
# [doc = " A cached time zone."] # [doc = ""] # [doc = " When there's a cached time zone that hasn't expired, then we return what's"] # [doc = " in the cache. This is because determining the time zone can be mildly"] # [doc = " expensive. For example, doing syscalls and potentially parsing TZif data."] # [doc = ""] # [doc = " We could use a `thread_local!` for this instead which may perhaps be"] # [doc = " faster."] # [doc = ""] # [doc = " Note that our cache here is somewhat simplistic because we lean on the"] # [doc = " fact that: 1) in the vast majority of cases, our platform specific code is"] # [doc = " limited to finding a time zone name, and 2) looking up a time zone name in"] # [doc = " `TimeZoneDatabase` has its own cache. The main cases this doesn't really"] # [doc = " cover are when we can't find a time zone name. In which case, we might be"] # [doc = " re-parsing POSIX TZ strings or TZif data unnecessarily. But it's not clear"] # [doc = " this matters much. It might matter more if we shrink our TTL though."] static CACHE : RwLock < Cache > = RwLock :: new (Cache :: empty ()) ;
};
}
