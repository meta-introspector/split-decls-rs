macro_rules! deps {
    () => {
        Options!();
        ProgramKind!();
    };
}

macro_rules! options {
    () => {
        deps!();
        mod options { mod ssh_command { use crate :: client :: blocking_io :: ssh :: { connect :: Options , ProgramKind } ; # [test] fn no_field_means_ssh () { assert_eq ! (Options :: default () . ssh_command () , "ssh") ; } # [test] fn command_field_determines_ssh_command () { assert_eq ! (Options { command : Some ("field-value" . into ()) , .. Default :: default () } . ssh_command () , "field-value") ; assert_eq ! (Options { command : Some ("field-value" . into ()) , kind : Some (ProgramKind :: TortoisePlink) , .. Default :: default () } . ssh_command () , "field-value") ; } # [test] fn kind_serves_as_fallback () { assert_eq ! (Options { kind : Some (ProgramKind :: TortoisePlink) , .. Default :: default () } . ssh_command () , "tortoiseplink.exe") ; } } }
    };
}

options!();