macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! DyldInfoCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DyldInfoCommand < E : Endian > { # [doc = " LC_DYLD_INFO or LC_DYLD_INFO_ONLY"] pub cmd : U32 < E > , # [doc = " sizeof(struct DyldInfoCommand)"] pub cmdsize : U32 < E > , # [doc = " file offset to rebase info"] pub rebase_off : U32 < E > , # [doc = " size of rebase info"] pub rebase_size : U32 < E > , # [doc = " file offset to binding info"] pub bind_off : U32 < E > , # [doc = " size of binding info"] pub bind_size : U32 < E > , # [doc = " file offset to weak binding info"] pub weak_bind_off : U32 < E > , # [doc = " size of weak binding info"] pub weak_bind_size : U32 < E > , # [doc = " file offset to lazy binding info"] pub lazy_bind_off : U32 < E > , # [doc = " size of lazy binding infs"] pub lazy_bind_size : U32 < E > , # [doc = " file offset to lazy binding info"] pub export_off : U32 < E > , # [doc = " size of lazy binding infs"] pub export_size : U32 < E > , }
    };
}

DyldInfoCommand!();