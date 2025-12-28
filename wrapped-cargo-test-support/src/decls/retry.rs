macro_rules! retry {
    () => {
        # [doc = " Helper to retry a function `n` times."] # [doc = ""] # [doc = " The function should return `Some` when it is ready."] # [track_caller] pub fn retry < F , R > (n : u32 , mut f : F) -> R where F : FnMut () -> Option < R > , { let mut count = 0 ; let start = std :: time :: Instant :: now () ; loop { if let Some (r) = f () { return r ; } count += 1 ; if count > n { panic ! ("test did not finish within {n} attempts ({:?} total)" , start . elapsed ()) ; } sleep_ms (100) ; } }
    };
}

retry!();