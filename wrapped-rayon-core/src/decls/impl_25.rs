macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
        ThreadPoolBuildError!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl ThreadPoolBuilder { # [doc = " Creates a scoped `ThreadPool` initialized using this configuration."] # [doc = ""] # [doc = " This is a convenience function for building a pool using [`std::thread::scope`]"] # [doc = " to spawn threads in a [`spawn_handler`]."] # [doc = " The threads in this pool will start by calling `wrapper`, which should"] # [doc = " do initialization and continue by calling `ThreadBuilder::run()`."] # [doc = ""] # [doc = " [`spawn_handler`]: Self::spawn_handler()"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " A scoped pool may be useful in combination with scoped thread-local variables."] # [doc = ""] # [doc = " ```ignore-wasm"] # [doc = " # use rayon_core as rayon;"] # [doc = ""] # [doc = " scoped_tls::scoped_thread_local!(static POOL_DATA: Vec<i32>);"] # [doc = ""] # [doc = " fn main() -> Result<(), rayon::ThreadPoolBuildError> {"] # [doc = "     let pool_data = vec![1, 2, 3];"] # [doc = ""] # [doc = "     // We haven't assigned any TLS data yet."] # [doc = "     assert!(!POOL_DATA.is_set());"] # [doc = ""] # [doc = "     rayon::ThreadPoolBuilder::new()"] # [doc = "         .build_scoped("] # [doc = "             // Borrow `pool_data` in TLS for each thread."] # [doc = "             |thread| POOL_DATA.set(&pool_data, || thread.run()),"] # [doc = "             // Do some work that needs the TLS data."] # [doc = "             |pool| pool.install(|| assert!(POOL_DATA.is_set())),"] # [doc = "         )?;"] # [doc = ""] # [doc = "     // Once we've returned, `pool_data` is no longer borrowed."] # [doc = "     drop(pool_data);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] pub fn build_scoped < W , F , R > (self , wrapper : W , with_pool : F) -> Result < R , ThreadPoolBuildError > where W : Fn (ThreadBuilder) + Sync , F : FnOnce (& ThreadPool) -> R , { std :: thread :: scope (| scope | { let pool = self . spawn_handler (| thread | { let mut builder = std :: thread :: Builder :: new () ; if let Some (name) = thread . name () { builder = builder . name (name . to_string ()) ; } if let Some (size) = thread . stack_size () { builder = builder . stack_size (size) ; } builder . spawn_scoped (scope , | | wrapper (thread)) ? ; Ok (()) }) . build () ? ; Ok (with_pool (& pool)) }) } }
    };
}

impl_25!()