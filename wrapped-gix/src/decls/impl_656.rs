macro_rules! deps {
    () => {
        Ssh!();
        Gitoxide!();
        Tree!();
        User!();
        Key!();
        Author!();
        Committer!();
        Http!();
        Commit!();
        Boolean!();
        Allow!();
        Any!();
        Core!();
        Pathspec!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        impl Gitoxide { # [doc = " The `gitoxide.allow` section."] pub const ALLOW : Allow = Allow ; # [doc = " The `gitoxide.author` section."] pub const AUTHOR : Author = Author ; # [doc = " The `gitoxide.core` section."] pub const CORE : Core = Core ; # [doc = " The `gitoxide.commit` section."] pub const COMMIT : Commit = Commit ; # [doc = " The `gitoxide.committer` section."] pub const COMMITTER : Committer = Committer ; # [doc = " The `gitoxide.credentials` section."] pub const CREDENTIALS : Credentials = Credentials ; # [doc = " The `gitoxide.http` section."] pub const HTTP : Http = Http ; # [doc = " The `gitoxide.https` section."] pub const HTTPS : Https = Https ; # [doc = " The `gitoxide.objects` section."] pub const OBJECTS : Objects = Objects ; # [doc = " The `gitoxide.ssh` section."] pub const SSH : Ssh = Ssh ; # [doc = " The `gitoxide.user` section."] pub const USER : User = User ; # [doc = " The `gitoxide.pathspec` section."] pub const PATHSPEC : Pathspec = Pathspec ; # [doc = " The `gitoxide.userAgent` Key."] pub const USER_AGENT : keys :: Any = keys :: Any :: new ("userAgent" , & config :: Tree :: GITOXIDE) . with_note ("The user agent presented on the git protocol layer, serving as fallback for when no `http.userAgent` is set" ,) ; # [doc = " The `gitoxide.tracePacket` Key."] pub const TRACE_PACKET : keys :: Boolean = keys :: Boolean :: new_boolean ("tracePacket" , & config :: Tree :: GITOXIDE) . with_environment_override ("GIT_TRACE_PACKET") ; # [doc = " The `gitoxide.parsePrecious` Key."] pub const PARSE_PRECIOUS : keys :: Boolean = keys :: Boolean :: new_boolean ("parsePrecious" , & config :: Tree :: GITOXIDE) . with_environment_override ("GIX_PARSE_PRECIOUS") ; }
    };
}

impl_656!()