macro_rules! SizedTypeProperties {
    () => {
        trait SizedTypeProperties : Sized { const IS_ZERO_SIZED : bool = mem :: size_of :: < Self > () == 0 ; const NEEDS_DROP : bool = mem :: needs_drop :: < Self > () ; }
    };
}

SizedTypeProperties!();