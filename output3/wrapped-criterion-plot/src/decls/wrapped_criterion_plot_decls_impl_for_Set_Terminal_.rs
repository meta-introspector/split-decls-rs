use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Set<Terminal> for Figure {
    /// Changes the output terminal
    ///
    /// **Note** By default, the terminal is set to `Svg`
    fn set(&mut self, terminal: Terminal) -> &mut Figure {
        self.terminal = terminal;
        self
    }
}
