use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Performs actions requested by the [`Parser`]
///
/// Actions in this case mean, for example, handling a CSI escape sequence describing cursor
/// movement, or simply printing characters to the screen.
///
/// The methods on this type correspond to actions described in
/// <http://vt100.net/emu/dec_ansi_parser>. I've done my best to describe them in
/// a useful way in my own words for completeness, but the site should be
/// referenced if something isn't clear. If the site disappears at some point in
/// the future, consider checking archive.org.
pub trait Perform {
    /// Draw a character to the screen and update states.
    fn print(&mut self, _c: char) {}
    /// Execute a C0 or C1 control function.
    fn execute(&mut self, _byte: u8) {}
    /// Invoked when a final character arrives in first part of device control string.
    ///
    /// The control function should be determined from the private marker, final character, and
    /// execute with a parameter list. A handler should be selected for remaining characters in the
    /// string; the handler function should subsequently be called by `put` for every character in
    /// the control string.
    ///
    /// The `ignore` flag indicates that more than two intermediates arrived and
    /// subsequent characters were ignored.
    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: u8) {}
    /// Pass bytes as part of a device control string to the handle chosen in `hook`. C0 controls
    /// will also be passed to the handler.
    fn put(&mut self, _byte: u8) {}
    /// Called when a device control string is terminated.
    ///
    /// The previously selected handler should be notified that the DCS has
    /// terminated.
    fn unhook(&mut self) {}
    /// Dispatch an operating system command.
    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}
    /// A final character has arrived for a CSI sequence
    ///
    /// The `ignore` flag indicates that either more than two intermediates arrived
    /// or the number of parameters exceeded the maximum supported length,
    /// and subsequent characters were ignored.
    fn csi_dispatch(
        &mut self,
        _params: &Params,
        _intermediates: &[u8],
        _ignore: bool,
        _action: u8,
    ) {
    }
    /// The final character of an escape sequence has arrived.
    ///
    /// The `ignore` flag indicates that more than two intermediates arrived and
    /// subsequent characters were ignored.
    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}
}
