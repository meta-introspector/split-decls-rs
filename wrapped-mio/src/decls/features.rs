macro_rules! deps {
    () => {
        Poll!();
        Waker!();
        Registry!();
    };
}

macro_rules! features {
    () => {
        deps!();
        pub mod features { # ! [doc = " # Mio's optional features."] # ! [doc = ""] # ! [doc = " This document describes the available features in Mio."] # ! [doc = ""] # ! [cfg_attr (feature = "os-poll" , doc = "## `os-poll` (enabled)")] # ! [cfg_attr (not (feature = "os-poll") , doc = "## `os-poll` (disabled)")] # ! [doc = ""] # ! [doc = " Mio by default provides only a shell implementation that `panic!`s the"] # ! [doc = " moment it is actually run. To run it requires OS support, this is"] # ! [doc = " enabled by activating the `os-poll` feature."] # ! [doc = ""] # ! [doc = " This makes `Poll`, `Registry` and `Waker` functional."] # ! [doc = ""] # ! [cfg_attr (feature = "os-ext" , doc = "## `os-ext` (enabled)")] # ! [cfg_attr (not (feature = "os-ext") , doc = "## `os-ext` (disabled)")] # ! [doc = ""] # ! [doc = " `os-ext` enables additional OS specific facilities. These facilities can"] # ! [doc = " be found in the `unix` and `windows` module."] # ! [doc = ""] # ! [cfg_attr (feature = "net" , doc = "## Network types (enabled)")] # ! [cfg_attr (not (feature = "net") , doc = "## Network types (disabled)")] # ! [doc = ""] # ! [doc = " The `net` feature enables networking primitives in the `net` module."] }
    };
}

features!();