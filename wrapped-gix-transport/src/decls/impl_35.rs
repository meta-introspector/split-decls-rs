macro_rules! deps {
    () => {
        ProgramKind!();
        Protocol!();
        SpawnProcessOnDemand!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl SpawnProcessOnDemand { pub (crate) fn new_ssh (url : gix_url :: Url , program : impl Into < OsString > , path : BString , ssh_kind : ssh :: ProgramKind , ssh_disallow_shell : bool , version : Protocol , trace : bool ,) -> SpawnProcessOnDemand { SpawnProcessOnDemand { url , path , ssh_cmd : Some ((program . into () , ssh_kind)) , envs : Default :: default () , ssh_disallow_shell , child : None , connection : None , desired_version : version , trace , } } fn new_local (path : BString , version : Protocol , trace : bool) -> SpawnProcessOnDemand { SpawnProcessOnDemand { url : gix_url :: Url :: from_parts (gix_url :: Scheme :: File , None , None , None , None , path . clone () , true) . expect ("valid url") , path , ssh_cmd : None , envs : if version != Protocol :: V1 { vec ! [("GIT_PROTOCOL" , format ! ("version={}" , version as usize))] } else { Default :: default () } , ssh_disallow_shell : false , child : None , connection : None , desired_version : version , trace , } } }
    };
}

impl_35!();