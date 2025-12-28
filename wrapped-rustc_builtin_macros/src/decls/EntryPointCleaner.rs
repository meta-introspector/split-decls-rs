macro_rules! EntryPointCleaner {
    () => {
        # [doc = " A folder used to remove any entry points (like fn main) because the harness"] # [doc = " coroutine will provide its own"] struct EntryPointCleaner < 'a > { sess : & 'a Session , depth : usize , def_site : Span , }
    };
}

EntryPointCleaner!();