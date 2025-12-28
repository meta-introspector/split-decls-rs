macro_rules! deps {
    () => {
        ProgramKind!();
        Connection!();
        Protocol!();
    };
}

macro_rules! SpawnProcessOnDemand {
    () => {
        deps!();
        # [doc = " A utility to spawn a helper process to actually transmit data, possibly over `ssh`."] # [doc = ""] # [doc = " It can only be instantiated using the local [`connect()`] or [ssh connect][super::ssh::connect()]."] pub struct SpawnProcessOnDemand { desired_version : Protocol , url : gix_url :: Url , path : BString , ssh_cmd : Option < (OsString , ssh :: ProgramKind) > , # [doc = " The environment variables to set in the invoked command."] envs : Vec < (& 'static str , String) > , ssh_disallow_shell : bool , connection : Option < Connection < Box < dyn std :: io :: Read + Send > , process :: ChildStdin > > , child : Option < process :: Child > , trace : bool , }
    };
}

SpawnProcessOnDemand!()