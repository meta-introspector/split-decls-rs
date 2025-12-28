macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { use super :: * ; # [test] # [cfg (any (target_os = "linux" , target_os = "macos" , target_os = "windows" , target_os = "freebsd" , target_os = "android"))] fn is_initialized_before_main () { assert ! (INITIALIZED . load (std :: sync :: atomic :: Ordering :: SeqCst)) ; } }
    };
}

tests!();