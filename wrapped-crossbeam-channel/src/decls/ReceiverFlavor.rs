macro_rules! deps {
    () => {
        Receiver!();
        Channel!();
    };
}

macro_rules! ReceiverFlavor {
    () => {
        deps!();
        # [doc = " Receiver flavors."] enum ReceiverFlavor < T > { # [doc = " Bounded channel based on a preallocated array."] Array (counter :: Receiver < flavors :: array :: Channel < T > >) , # [doc = " Unbounded channel implemented as a linked list."] List (counter :: Receiver < flavors :: list :: Channel < T > >) , # [doc = " Zero-capacity channel."] Zero (counter :: Receiver < flavors :: zero :: Channel < T > >) , # [doc = " The after flavor."] At (Arc < flavors :: at :: Channel >) , # [doc = " The tick flavor."] Tick (Arc < flavors :: tick :: Channel >) , # [doc = " The never flavor."] Never (flavors :: never :: Channel < T >) , }
    };
}

ReceiverFlavor!()