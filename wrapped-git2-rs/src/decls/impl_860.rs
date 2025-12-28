macro_rules! deps {
    () => {
        Reference!();
        WorktreeAddOptions!();
    };
}

macro_rules! impl_860 {
    () => {
        deps!();
        impl < 'a > WorktreeAddOptions < 'a > { # [doc = " Creates a default set of add options."] # [doc = ""] # [doc = " By default this will not lock the worktree"] pub fn new () -> WorktreeAddOptions < 'a > { unsafe { let mut raw = mem :: zeroed () ; assert_eq ! (raw :: git_worktree_add_options_init (& mut raw , raw :: GIT_WORKTREE_ADD_OPTIONS_VERSION) , 0) ; WorktreeAddOptions { raw , _marker : marker :: PhantomData , } } } # [doc = " If enabled, this will cause the newly added worktree to be locked"] pub fn lock (& mut self , enabled : bool) -> & mut WorktreeAddOptions < 'a > { self . raw . lock = enabled as c_int ; self } # [doc = " If enabled, this will checkout the existing branch matching the worktree name."] pub fn checkout_existing (& mut self , enabled : bool) -> & mut WorktreeAddOptions < 'a > { self . raw . checkout_existing = enabled as c_int ; self } # [doc = " reference to use for the new worktree HEAD"] pub fn reference (& mut self , reference : Option < & 'a Reference < '_ > > ,) -> & mut WorktreeAddOptions < 'a > { self . raw . reference = if let Some (reference) = reference { reference . raw () } else { ptr :: null_mut () } ; self } # [doc = " Get a set of raw add options to be used with `git_worktree_add`"] pub fn raw (& self) -> * const raw :: git_worktree_add_options { & self . raw } }
    };
}

impl_860!()