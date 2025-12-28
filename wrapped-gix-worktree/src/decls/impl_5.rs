macro_rules! deps {
    () => {
        PathIdMapping!();
        State!();
        Stack!();
        Statistics!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        # [doc = " Initialization"] impl Stack { # [doc = " Create a new instance with `worktree_root` being the base for all future paths we match."] # [doc = " `state` defines the capabilities of the cache."] # [doc = " The `case` configures attribute and exclusion case sensitivity at *query time*, which should match the case that"] # [doc = " `state` might be configured with."] # [doc = " `buf` is used when reading files, and `id_mappings` should have been created with [`State::id_mappings_from_index()`]."] pub fn new (worktree_root : impl Into < PathBuf > , state : State , case : gix_glob :: pattern :: Case , buf : Vec < u8 > , id_mappings : Vec < PathIdMapping > ,) -> Self { let root = worktree_root . into () ; Stack { stack : gix_fs :: Stack :: new (root) , state , case , buf , id_mappings , statistics : Statistics :: default () , } } # [doc = " Create a new stack that takes into consideration the `ignore_case` result of a filesystem probe in `root`. It takes a configured"] # [doc = " `state` to control what it can do, while initializing attribute or ignore files that are to be queried from the ODB using"] # [doc = " `index` and `path_backing`."] # [doc = ""] # [doc = " This is the easiest way to correctly setup a stack."] pub fn from_state_and_ignore_case (root : impl Into < PathBuf > , ignore_case : bool , state : State , index : & gix_index :: State , path_backing : & gix_index :: PathStorageRef ,) -> Self { let case = if ignore_case { gix_glob :: pattern :: Case :: Fold } else { gix_glob :: pattern :: Case :: Sensitive } ; let attribute_files = state . id_mappings_from_index (index , path_backing , case) ; Stack :: new (root , state , case , Vec :: with_capacity (512) , attribute_files) } }
    };
}

impl_5!()