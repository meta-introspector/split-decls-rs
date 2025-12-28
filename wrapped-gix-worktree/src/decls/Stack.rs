macro_rules! deps {
    () => {
        Statistics!();
        State!();
        PathIdMapping!();
    };
}

macro_rules! Stack {
    () => {
        deps!();
        # [doc = " A cache for efficiently executing operations on directories and files which are encountered in sorted order."] # [doc = " That way, these operations can be re-used for subsequent invocations in the same directory."] # [doc = ""] # [doc = " This cache can be configured to create directories efficiently, read git-ignore files and git-attribute files,"] # [doc = " in any combination."] # [doc = ""] # [doc = " A cache for directory creation to reduce the amount of stat calls when creating"] # [doc = " directories safely, that is without following symlinks that might be on the way."] # [doc = ""] # [doc = " As a special case, it offers a 'prefix' which (by itself) is assumed to exist and may contain symlinks."] # [doc = " Everything past that prefix boundary must not contain a symlink. We do this by allowing any input path."] # [doc = ""] # [doc = " Another added benefit is its ability to store the path of full path of the entry to which leading directories"] # [doc = " are to be created to avoid allocating memory."] # [doc = ""] # [doc = " For this to work, it remembers the last 'good' path to a directory and assumes that all components of it"] # [doc = " are still valid, too."] # [doc = " As directories are created, the cache will be adjusted to reflect the latest seen directory."] # [doc = ""] # [doc = " The caching is only useful if consecutive calls to create a directory are using a sorted list of entries."] # [derive (Clone)] pub struct Stack { stack : gix_fs :: Stack , # [doc = " tells us what to do as we change paths."] state : stack :: State , # [doc = " A buffer used when reading attribute or ignore files or their respective objects from the object database."] buf : Vec < u8 > , # [doc = " If case folding should happen when looking up attributes or exclusions."] case : gix_glob :: pattern :: Case , # [doc = " A lookup table for object ids to read from in some situations when looking up attributes or exclusions."] id_mappings : Vec < PathIdMapping > , statistics : stack :: Statistics , }
    };
}

Stack!()