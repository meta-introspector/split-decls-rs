// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_passes/src/debugger_visualizer.rs
// Error: expected square brackets
// Problematic line: line 13


use crate::errors::{DebugVisualizerInvalid, DebugVisualizerUnreadable};

impl DebuggerVisualizerCollector<'_> {
    fn check_for_debugger_visualizer(&mut self, attr: &Attribute) {
        if attr.has_name(sym::debugger_visualizer) {
            let Some(hints) = attr.meta_item_list() else {
