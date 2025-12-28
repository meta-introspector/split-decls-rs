macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl Default for Command { fn default () -> Self { Self { name : Default :: default () , long_flag : Default :: default () , short_flag : Default :: default () , display_name : Default :: default () , bin_name : Default :: default () , author : Default :: default () , version : Default :: default () , long_version : Default :: default () , about : Default :: default () , long_about : Default :: default () , before_help : Default :: default () , before_long_help : Default :: default () , after_help : Default :: default () , after_long_help : Default :: default () , aliases : Default :: default () , short_flag_aliases : Default :: default () , long_flag_aliases : Default :: default () , usage_str : Default :: default () , usage_name : Default :: default () , help_str : Default :: default () , disp_ord : Default :: default () , # [cfg (feature = "help")] template : Default :: default () , settings : Default :: default () , g_settings : Default :: default () , args : Default :: default () , subcommands : Default :: default () , groups : Default :: default () , current_help_heading : Default :: default () , current_disp_ord : Some (0) , subcommand_value_name : Default :: default () , subcommand_heading : Default :: default () , external_value_parser : Default :: default () , long_help_exists : false , deferred : None , # [cfg (feature = "unstable-ext")] ext : Default :: default () , app_ext : Default :: default () , } } }
    };
}

impl_86!()