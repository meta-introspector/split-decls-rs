macro_rules! deps {
    () => {
        Remote!();
        RemoteConnection!();
        Buf!();
        RemoteHead!();
        Error!();
    };
}

macro_rules! impl_659 {
    () => {
        deps!();
        impl < 'repo , 'connection , 'cb > RemoteConnection < 'repo , 'connection , 'cb > { # [doc = " Check whether the remote is (still) connected"] pub fn connected (& mut self) -> bool { self . remote . connected () } # [doc = " Get the remote repository's reference advertisement list."] # [doc = ""] # [doc = " This list is available as soon as the connection to"] # [doc = " the remote is initiated and it remains available after disconnecting."] pub fn list (& self) -> Result < & [RemoteHead < '_ >] , Error > { self . remote . list () } # [doc = " Get the remote's default branch."] # [doc = ""] # [doc = " This default branch is available as soon as the connection to the remote"] # [doc = " is initiated and it remains available after disconnecting."] pub fn default_branch (& self) -> Result < Buf , Error > { self . remote . default_branch () } # [doc = " access remote bound to this connection"] pub fn remote (& mut self) -> & mut Remote < 'repo > { self . remote } }
    };
}

impl_659!()