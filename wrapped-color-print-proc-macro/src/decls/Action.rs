macro_rules! deps {
    () => {
        Change!();
    };
}

macro_rules! Action {
    () => {
        deps!();
        # [doc = " The action to be performed on a given color/style attribute in order to reach a new state."] # [derive (Debug , PartialEq)] pub enum Action < T > { # [doc = " Nothing has to be done, because this value was never modified."] None , # [doc = " This attribute has to be kept the same."] # [doc = " With the terminfo implementation, it's not possible to reset each style/color"] # [doc = " independently, so we have to keep track of the values, even with the `Keep` variant."] Keep (T) , # [doc = " This attribute value has to be changed."] Change (T) , }
    };
}

Action!()