macro_rules! RngReader {
    () => {
        # [doc = " Adapter to support [`std::io::Read`] over a [`TryRngCore`]"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::{io, io::Read};"] # [doc = " use std::fs::File;"] # [doc = " use rand::{rngs::OsRng, RngReader};"] # [doc = ""] # [doc = " io::copy("] # [doc = "     &mut RngReader(OsRng).take(100),"] # [doc = "     &mut File::create(\"/tmp/random.bytes\").unwrap()"] # [doc = " ).unwrap();"] # [doc = " ```"] # [cfg (feature = "std")] pub struct RngReader < R : TryRngCore > (pub R) ;
    };
}

RngReader!();