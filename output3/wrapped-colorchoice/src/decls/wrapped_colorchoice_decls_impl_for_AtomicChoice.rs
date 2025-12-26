use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AtomicChoice {
    pub(crate) const fn new() -> Self {
        Self(AtomicUsize::new(Self::from_choice(ColorChoice::Auto)))
    }
    pub(crate) fn get(&self) -> ColorChoice {
        let choice = self.0.load(Ordering::SeqCst);
        Self::to_choice(choice).expect("Only `ColorChoice` values can be `set`")
    }
    pub(crate) fn set(&self, choice: ColorChoice) {
        let choice = Self::from_choice(choice);
        self.0.store(choice, Ordering::SeqCst);
    }
    const fn from_choice(choice: ColorChoice) -> usize {
        match choice {
            ColorChoice::Auto => 0,
            ColorChoice::AlwaysAnsi => 1,
            ColorChoice::Always => 2,
            ColorChoice::Never => 3,
        }
    }
    const fn to_choice(choice: usize) -> Option<ColorChoice> {
        match choice {
            0 => Some(ColorChoice::Auto),
            1 => Some(ColorChoice::AlwaysAnsi),
            2 => Some(ColorChoice::Always),
            3 => Some(ColorChoice::Never),
            _ => None,
        }
    }
}
