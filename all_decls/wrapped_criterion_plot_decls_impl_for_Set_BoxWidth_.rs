use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Set<BoxWidth> for Figure {
    /// Changes the box width of all the box related plots (bars, candlesticks, etc)
    ///
    /// **Note** The default value is 0
    ///
    /// # Panics
    ///
    /// Panics if `width` is a negative value
    fn set(&mut self, width: BoxWidth) -> &mut Figure {
        let width = width.0;
        assert!(width >= 0.);
        self.box_width = Some(width);
        self
    }
}
