macro_rules! CharEscape {
    () => {
        # [doc = " Represents a character escape code in a type-safe manner."] pub enum CharEscape { # [doc = " An escaped quote `\"`"] Quote , # [doc = " An escaped reverse solidus `\\`"] ReverseSolidus , # [doc = " An escaped solidus `/`"] Solidus , # [doc = " An escaped backspace character (usually escaped as `\\b`)"] Backspace , # [doc = " An escaped form feed character (usually escaped as `\\f`)"] FormFeed , # [doc = " An escaped line feed character (usually escaped as `\\n`)"] LineFeed , # [doc = " An escaped carriage return character (usually escaped as `\\r`)"] CarriageReturn , # [doc = " An escaped tab character (usually escaped as `\\t`)"] Tab , # [doc = " An escaped ASCII plane control character (usually escaped as"] # [doc = " `\\u00XX` where `XX` are two hex characters)"] AsciiControl (u8) , }
    };
}

CharEscape!();