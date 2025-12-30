// Generated macro for FutureObj (struct)
macro_rules! Depcrate_future_objFutureObj {
() => {
// Module: crate::future_obj
// Provides: {"FutureObj"}
// Dependencies: {}
# [doc = " A custom trait object for polling futures, roughly akin to"] # [doc = " `Box<dyn Future<Output = T> + Send + 'a>`."] # [doc = ""] # [doc = " This custom trait object was introduced as currently it is not possible to"] # [doc = " take `dyn Trait` by value and `Box<dyn Trait>` is not available in no_std"] # [doc = " contexts."] # [doc = ""] # [doc = " You should generally not need to use this type outside of `no_std` or when"] # [doc = " implementing `Spawn`, consider using `BoxFuture` instead."] pub struct FutureObj < 'a , T > (LocalFutureObj < 'a , T >) ;
};
}
