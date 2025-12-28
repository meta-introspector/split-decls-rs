macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! set_verify_owner_validation {
    () => {
        deps!();
        # [doc = " Set whether or not to verify ownership before performing a repository."] # [doc = " Enabled by default, but disabling this can lead to code execution vulnerabilities."] pub unsafe fn set_verify_owner_validation (enabled : bool) -> Result < () , Error > { crate :: init () ; let error = raw :: git_libgit2_opts (raw :: GIT_OPT_SET_OWNER_VALIDATION as libc :: c_int , enabled as libc :: c_int ,) ; debug_assert ! (error >= 0) ; Ok (()) }
    };
}

set_verify_owner_validation!();