macro_rules! deps {
    () => {
        Binary!();
        Diff!();
        Program!();
        Algorithm!();
        UnsignedInteger!();
        Ignore!();
        Tree!();
        Renames!();
        SubSectionRequirement!();
    };
}

macro_rules! impl_629 {
    () => {
        deps!();
        impl Diff { # [doc = " The `diff.algorithm` key."] pub const ALGORITHM : Algorithm = Algorithm :: new_with_validate ("algorithm" , & config :: Tree :: DIFF , validate :: Algorithm) . with_deviation ("'patience' diff is not implemented and can default to 'histogram' if lenient config is used, and defaults to histogram if unset for fastest and best results") ; # [doc = " The `diff.renameLimit` key."] pub const RENAME_LIMIT : keys :: UnsignedInteger = keys :: UnsignedInteger :: new_unsigned_integer ("renameLimit" , & config :: Tree :: DIFF ,) . with_note ("The limit is actually squared, so 1000 stands for up to 1 million diffs if fuzzy rename tracking is enabled" ,) ; # [doc = " The `diff.ignoreSubmodules` key."] pub const IGNORE_SUBMODULES : Ignore = Ignore :: new_with_validate ("ignoreSubmodules" , & config :: Tree :: DIFF , validate :: Ignore) . with_note ("This setting affects only the submodule status, and thus the repository status in general.") ; # [doc = " The `diff.renames` key."] pub const RENAMES : Renames = Renames :: new_renames ("renames" , & config :: Tree :: DIFF) ; # [doc = " The `diff.<driver>.command` key."] pub const DRIVER_COMMAND : keys :: Program = keys :: Program :: new_program ("command" , & config :: Tree :: DIFF) . with_subsection_requirement (Some (SubSectionRequirement :: Parameter ("driver"))) ; # [doc = " The `diff.<driver>.textconv` key."] pub const DRIVER_TEXTCONV : keys :: Program = keys :: Program :: new_program ("textconv" , & config :: Tree :: DIFF) . with_subsection_requirement (Some (SubSectionRequirement :: Parameter ("driver"))) ; # [doc = " The `diff.<driver>.algorithm` key."] pub const DRIVER_ALGORITHM : Algorithm = Algorithm :: new_with_validate ("algorithm" , & config :: Tree :: DIFF , validate :: Algorithm) . with_subsection_requirement (Some (SubSectionRequirement :: Parameter ("driver"))) ; # [doc = " The `diff.<driver>.binary` key."] pub const DRIVER_BINARY : Binary = Binary :: new_with_validate ("binary" , & config :: Tree :: DIFF , validate :: Binary) . with_subsection_requirement (Some (SubSectionRequirement :: Parameter ("driver"))) ; # [doc = " The `diff.external` key."] pub const EXTERNAL : keys :: Program = keys :: Program :: new_program ("external" , & config :: Tree :: DIFF) . with_environment_override ("GIT_EXTERNAL_DIFF") ; }
    };
}

impl_629!();