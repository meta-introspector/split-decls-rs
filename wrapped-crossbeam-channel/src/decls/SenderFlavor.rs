macro_rules! deps {
    () => {
        Channel!();
        Sender!();
    };
}

macro_rules! SenderFlavor {
    () => {
        deps!();
        # [doc = " Sender flavors."] enum SenderFlavor < T > { # [doc = " Bounded channel based on a preallocated array."] Array (counter :: Sender < flavors :: array :: Channel < T > >) , # [doc = " Unbounded channel implemented as a linked list."] List (counter :: Sender < flavors :: list :: Channel < T > >) , # [doc = " Zero-capacity channel."] Zero (counter :: Sender < flavors :: zero :: Channel < T > >) , }
    };
}

SenderFlavor!();