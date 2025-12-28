macro_rules! deps {
    () => {
        Note!();
        RefStore!();
        Options!();
        OdbHandle!();
        Cache!();
        ModulesFileStorage!();
        CommitsStorage!();
        IndexStorage!();
    };
}

macro_rules! Repository {
    () => {
        deps!();
        # [doc = " A thread-local handle to interact with a repository from a single thread."] # [doc = ""] # [doc = " It is `Send`, but **not** `Sync` - for the latter you can convert it using"] # [doc = " [`Repository::into_sync()`]."] # [doc = ""] # [doc = " Note that it clones itself so that it is empty, requiring the user to configure each clone separately, specifically"] # [doc = " and explicitly. This is to have the fastest-possible default configuration available by default, but allow"] # [doc = " those who experiment with workloads to get speed boosts of 2x or more."] # [doc = ""] # [doc = " ### `Send` only with `parallel` feature"] # [doc = ""] # [doc = " When built with `default-features = false`, this type is **not** `Send`."] # [doc = " The minimal feature set to activate `Send` is `features = [\"parallel\"]`."] pub struct Repository { # [doc = " A ref store with shared ownership (or the equivalent of it)."] pub refs : crate :: RefStore , # [doc = " A way to access objects."] pub objects : crate :: OdbHandle , pub (crate) work_tree : Option < PathBuf > , # [doc = " The path to the resolved common directory if this is a linked worktree repository or it is otherwise set."] pub (crate) common_dir : Option < PathBuf > , # [doc = " A free-list of reusable object backing buffers"] pub (crate) bufs : Option < RefCell < Vec < Vec < u8 > > > > , # [doc = " A pre-assembled selection of often-accessed configuration values for quick access."] pub (crate) config : crate :: config :: Cache , # [doc = " the options obtained when instantiating this repository."] # [doc = ""] # [doc = " Particularly useful when following linked worktrees and instantiating new equally configured worktree repositories."] pub (crate) options : crate :: open :: Options , # [cfg (feature = "index")] pub (crate) index : crate :: worktree :: IndexStorage , # [cfg (feature = "attributes")] pub (crate) modules : crate :: submodule :: ModulesFileStorage , pub (crate) shallow_commits : crate :: shallow :: CommitsStorage , }
    };
}

Repository!()