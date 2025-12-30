// Generated macro for tests (module)
macro_rules! Depcrate_client_blocking_io_filetests {
() => {
// Module: crate::client::blocking_io::file
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { mod ssh { mod connect { use crate :: { client :: blocking_io :: ssh , Protocol } ; # [test] fn path () { for (url , expected) in [("ssh://host.xy/~/repo" , "~/repo") , ("ssh://host.xy/~username/repo" , "~username/repo") , ("user@host.xy:/username/repo" , "/username/repo") , ("user@host.xy:username/repo" , "username/repo") , ("user@host.xy:../username/repo" , "../username/repo") , ("user@host.xy:~/repo" , "~/repo") ,] { let url = gix_url :: parse ((* url) . into ()) . expect ("valid url") ; let cmd = ssh :: connect (url , Protocol :: V1 , Default :: default () , false) . expect ("parse success") ; assert_eq ! (cmd . path , expected , "the path will be substituted by the remote shell") ; } } # [test] fn ambiguous_host_disallowed () { for url in ["ssh://-oProxyCommand=open$IFS-aCalculator/foo" , "user@-oProxyCommand=open$IFS-aCalculator:username/repo" ,] { let url = gix_url :: parse ((* url) . into ()) . expect ("valid url") ; let options = ssh :: connect :: Options { command : Some ("unrecognized" . into ()) , disallow_shell : false , kind : None , } ; assert ! (matches ! (ssh :: connect (url , Protocol :: V1 , options , false) , Err (ssh :: Error :: AmbiguousHostName { host }) if host == "-oProxyCommand=open$IFS-aCalculator" ,)) ; } } } } }
};
}
