macro_rules! deps {
    () => {
        HunkCB!();
        DiffHunk!();
        DeltaCB!();
        DiffDelta!();
        ApplyOptions!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < 'cb > ApplyOptions < 'cb > { # [doc = " Creates a new set of empty options (zeroed)."] pub fn new () -> Self { let mut opts = Self { raw : unsafe { mem :: zeroed () } , hunk_cb : None , delta_cb : None , } ; assert_eq ! (unsafe { raw :: git_apply_options_init (& mut opts . raw , raw :: GIT_APPLY_OPTIONS_VERSION) } , 0) ; opts } fn flag (& mut self , opt : raw :: git_apply_flags_t , val : bool) -> & mut Self { let opt = opt as u32 ; if val { self . raw . flags |= opt ; } else { self . raw . flags &= ! opt ; } self } # [doc = " Don't actually make changes, just test that the patch applies."] pub fn check (& mut self , check : bool) -> & mut Self { self . flag (raw :: GIT_APPLY_CHECK , check) } # [doc = " When applying a patch, callback that will be made per hunk."] pub fn hunk_callback < F > (& mut self , cb : F) -> & mut Self where F : FnMut (Option < DiffHunk < '_ > >) -> bool + 'cb , { self . hunk_cb = Some (Box :: new (cb) as Box < HunkCB < 'cb > >) ; self . raw . hunk_cb = Some (hunk_cb_c) ; self . raw . payload = self as * mut _ as * mut _ ; self } # [doc = " When applying a patch, callback that will be made per delta (file)."] pub fn delta_callback < F > (& mut self , cb : F) -> & mut Self where F : FnMut (Option < DiffDelta < '_ > >) -> bool + 'cb , { self . delta_cb = Some (Box :: new (cb) as Box < DeltaCB < 'cb > >) ; self . raw . delta_cb = Some (delta_cb_c) ; self . raw . payload = self as * mut _ as * mut _ ; self } # [doc = " Pointer to a raw git_stash_apply_options"] pub unsafe fn raw (& mut self) -> * const raw :: git_apply_options { & self . raw as * const _ } }
    };
}

impl_194!()