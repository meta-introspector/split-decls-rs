macro_rules! find_subcommand_with_path {
    () => {
        # [doc = " Finds the subcommand [`clap::Command`] from the given [`clap::Command`] with the given path."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** `path` should not contain the root `bin_name`."] # [doc = ""] # [doc = " </div>"] pub fn find_subcommand_with_path < 'cmd > (p : & 'cmd Command , path : Vec < & str >) -> & 'cmd Command { let mut cmd = p ; for sc in path { cmd = cmd . find_subcommand (sc) . unwrap () ; } cmd }
    };
}

find_subcommand_with_path!()