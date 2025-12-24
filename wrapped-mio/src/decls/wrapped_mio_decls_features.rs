use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod features {
    //! # Mio's optional features.
    //!
    //! This document describes the available features in Mio.
    //!
    #![cfg_attr(feature = "os-poll", doc = "## `os-poll` (enabled)")]
    #![cfg_attr(not(feature = "os-poll"), doc = "## `os-poll` (disabled)")]
    //!
    //! Mio by default provides only a shell implementation that `panic!`s the
    //! moment it is actually run. To run it requires OS support, this is
    //! enabled by activating the `os-poll` feature.
    //!
    //! This makes `Poll`, `Registry` and `Waker` functional.
    //!
    #![cfg_attr(feature = "os-ext", doc = "## `os-ext` (enabled)")]
    #![cfg_attr(not(feature = "os-ext"), doc = "## `os-ext` (disabled)")]
    //!
    //! `os-ext` enables additional OS specific facilities. These facilities can
    //! be found in the `unix` and `windows` module.
    //!
    #![cfg_attr(feature = "net", doc = "## Network types (enabled)")]
    #![cfg_attr(not(feature = "net"), doc = "## Network types (disabled)")]
    //!
    //! The `net` feature enables networking primitives in the `net` module.
}
