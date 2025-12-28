macro_rules! deps {
    () => {
        DeserializeError!();
    };
}

macro_rules! read_version {
    () => {
        deps!();
        # [doc = " Reads a version number from the beginning of the given slice and confirms"] # [doc = " that is matches the expected version number given. If the slice is too"] # [doc = " small or if the version numbers aren't equivalent, this returns an error."] # [doc = ""] # [doc = " Upon success, the total number of bytes read is returned."] # [doc = ""] # [doc = " N.B. Currently, we require that the version number is exactly equivalent."] # [doc = " In the future, if we bump the version number without a semver bump, then"] # [doc = " we'll need to relax this a bit and support older versions."] pub (crate) fn read_version (slice : & [u8] , expected_version : u32 ,) -> Result < usize , DeserializeError > { let (n , nr) = try_read_u32 (slice , "version") ? ; assert_eq ! (nr , write_version_len ()) ; if n != expected_version { return Err (DeserializeError :: version_mismatch (expected_version , n)) ; } Ok (nr) }
    };
}

read_version!()