macro_rules! no_carriage_return {
    () => {
        # [inline] fn no_carriage_return (line : & str) -> & str { line . strip_suffix ('\r') . unwrap_or (line) }
    };
}

no_carriage_return!();