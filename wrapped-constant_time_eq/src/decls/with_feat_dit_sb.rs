macro_rules! with_feat_dit_sb {
    () => {
        # [doc = " Wraps code with DIT when `FEAT_DIT` and `FEAT_SB` were detected."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `FEAT_DIT` and `FEAT_SB` must have been detected."] # [inline] # [target_feature (enable = "dit,sb")] unsafe fn with_feat_dit_sb < T , F > (f : F) -> T where F : FnOnce () -> T , { struct Guard { dit : u64 , } impl Drop for Guard { # [inline] fn drop (& mut self) { unsafe { wsr64_dit (self . dit) } ; } } let _guard = Guard { dit : unsafe { rsr64_dit () } , } ; unsafe { enable_dit () } ; unsafe { speculation_barrier () } ; f () }
    };
}

with_feat_dit_sb!()