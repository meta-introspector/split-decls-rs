#![feature(proc_macro_hygiene)]

use include_dir_macro::include_dir;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

// Standard library imports

use std::boxed::Box; // Required for Box<dyn ParseResult>

mod macros;
pub use macros::box_dyn_parse_result_base::*;

// Re-exports from macros module
pub use macros::box_dyn_parse_result_base_return_t::*;

// Re-exports from tracker module
pub use tracker::LibMacroRuleTracker;

// Argument macros (re-added)
#[macro_export]
macro_rules! LibMacroRuleTrackerTokenStreamArgT {
    () => { &rustc_ast::tokenstream::TokenStream };
}

#[macro_export]
macro_rules! LibMacroRuleTrackerSpanArgT {
    () => { rustc_span::Span };
}

#[macro_export]
macro_rules! MTrackerTrait {
    () => {
        $crate::tracker::LibMacroRuleTracker
    };
}

#[macro_export]
macro_rules! DynMTrackerTrait {
    () => {
        dyn $crate::tracker::LibMacroRuleTracker<Failure = Box<dyn $crate::parse_result::ParseResultBase<()>>>
    };
}

#[macro_export]
macro_rules! ImplMTrackerTrait {
    ($for_type:ty { $($body:tt)* }) => {
        impl $crate::tracker::LibMacroRuleTracker for $for_type {
            $($body)*
        }
    };
}



// Rustc imports
use rustc_ast::token::{self, Token};
use rustc_ast::tokenstream::TokenStream;
use rustc_data_structures::fx::FxHashMap;
use rustc_errors::{DiagCtxtHandle, ErrorGuaranteed, PResult};
use rustc_parse::parser::Recovery;
use rustc_session::parse::ParseSess;
use rustc_span::{hygiene::ExpnKind, Ident, MacroRulesNormalizedIdent, Span, Symbol};
use lib_matcher_loc::MatcherLoc; // From lib-matcher-loc crate
use lib_token_tree::{TokenTree};
use tracing::{debug, instrument, trace, trace_span};
use rustc_ast_pretty::pprust;
use rustc_ast::{NodeId, Safety};


// Internal modules
mod macro_rule;
pub mod parse_result;
pub mod ok_parse;
pub mod err_parse;
pub mod tracker;
pub mod dummy_tracker;

// Re-exports
pub use macro_rule::*;
pub use parse_result::*;
pub use ok_parse::*;
pub use err_parse::*;

pub use dummy_tracker::*;


