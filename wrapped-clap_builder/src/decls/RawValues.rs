macro_rules! deps {
    () => {
        OsStr!();
        Iter!();
    };
}

macro_rules! RawValues {
    () => {
        deps!();
        # [doc = " Iterate over raw argument values via [`ArgMatches::get_raw`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(unix)] {"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::{Command, arg, value_parser};"] # [doc = " use std::ffi::OsString;"] # [doc = " use std::os::unix::ffi::{OsStrExt,OsStringExt};"] # [doc = ""] # [doc = " let m = Command::new(\"utf8\")"] # [doc = "     .arg(arg!(<arg> \"some arg\")"] # [doc = "         .value_parser(value_parser!(OsString)))"] # [doc = "     .get_matches_from(vec![OsString::from(\"myprog\"),"] # [doc = "                             // \"Hi {0xe9}!\""] # [doc = "                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);"] # [doc = " assert_eq!("] # [doc = "     &*m.get_raw(\"arg\")"] # [doc = "         .unwrap()"] # [doc = "         .next().unwrap()"] # [doc = "         .as_bytes(),"] # [doc = "     [b'H', b'i', b' ', 0xe9, b'!']"] # [doc = " );"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct RawValues < 'a > { # [allow (clippy :: type_complexity)] iter : Map < Flatten < Iter < 'a , Vec < OsString > > > , fn (& OsString) -> & OsStr > , len : usize , }
    };
}

RawValues!();