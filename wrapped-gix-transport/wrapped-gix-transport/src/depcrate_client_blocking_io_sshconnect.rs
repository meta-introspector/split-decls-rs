// Generated macro for connect (function)
macro_rules! Depcrate_client_blocking_io_sshconnect {
() => {
// Module: crate::client::blocking_io::ssh
// Provides: {"connect"}
// Dependencies: {}
# [doc = " Connect to `host` using the ssh program to obtain data from the repository at `path` on the remote."] # [doc = ""] # [doc = " The optional `user` identifies the user's account to which to connect, while `port` allows to specify non-standard"] # [doc = " ssh ports."] # [doc = ""] # [doc = " The `desired_version` is the preferred protocol version when establishing the connection, but note that it can be"] # [doc = " downgraded by servers not supporting it."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] # [allow (clippy :: result_large_err)] pub fn connect (url : Url , desired_version : Protocol , options : connect :: Options , trace : bool ,) -> Result < SpawnProcessOnDemand , Error > { if url . scheme != gix_url :: Scheme :: Ssh || url . host () . is_none () { return Err (Error :: UnsupportedScheme (url)) ; } let ssh_cmd = options . ssh_command () ; let kind = determine_client_kind (options . kind , ssh_cmd , & url , options . disallow_shell) ? ; let path = gix_url :: expand_path :: for_shell (url . path . clone ()) ; Ok (SpawnProcessOnDemand :: new_ssh (url , ssh_cmd , path , kind , options . disallow_shell , desired_version , trace ,)) }
};
}
