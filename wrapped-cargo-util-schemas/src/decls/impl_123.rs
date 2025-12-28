macro_rules! deps {
    () => {
        TomlProfile!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl TomlProfile { # [doc = " Overwrite self's values with the given profile."] pub fn merge (& mut self , profile : & Self) { if let Some (v) = & profile . opt_level { self . opt_level = Some (v . clone ()) ; } if let Some (v) = & profile . lto { self . lto = Some (v . clone ()) ; } if let Some (v) = & profile . codegen_backend { self . codegen_backend = Some (v . clone ()) ; } if let Some (v) = profile . codegen_units { self . codegen_units = Some (v) ; } if let Some (v) = profile . debug { self . debug = Some (v) ; } if let Some (v) = profile . debug_assertions { self . debug_assertions = Some (v) ; } if let Some (v) = & profile . split_debuginfo { self . split_debuginfo = Some (v . clone ()) ; } if let Some (v) = profile . rpath { self . rpath = Some (v) ; } if let Some (v) = & profile . panic { self . panic = Some (v . clone ()) ; } if let Some (v) = profile . overflow_checks { self . overflow_checks = Some (v) ; } if let Some (v) = profile . incremental { self . incremental = Some (v) ; } if let Some (v) = & profile . rustflags { self . rustflags = Some (v . clone ()) ; } if let Some (other_package) = & profile . package { match & mut self . package { Some (self_package) => { for (spec , other_pkg_profile) in other_package { match self_package . get_mut (spec) { Some (p) => p . merge (other_pkg_profile) , None => { self_package . insert (spec . clone () , other_pkg_profile . clone ()) ; } } } } None => self . package = Some (other_package . clone ()) , } } if let Some (other_bo) = & profile . build_override { match & mut self . build_override { Some (self_bo) => self_bo . merge (other_bo) , None => self . build_override = Some (other_bo . clone ()) , } } if let Some (v) = & profile . inherits { self . inherits = Some (v . clone ()) ; } if let Some (v) = & profile . dir_name { self . dir_name = Some (v . clone ()) ; } if let Some (v) = & profile . strip { self . strip = Some (v . clone ()) ; } if let Some (v) = & profile . trim_paths { self . trim_paths = Some (v . clone ()) } if let Some (v) = profile . hint_mostly_unused { self . hint_mostly_unused = Some (v) ; } } }
    };
}

impl_123!();