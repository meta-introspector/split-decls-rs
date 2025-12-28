macro_rules! deps {
    () => {
        Storage!();
        QueueInner!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl < T , S , S2 > PartialEq < QueueInner < T , S2 > > for QueueInner < T , S > where T : PartialEq , S : Storage , S2 : Storage , { fn eq (& self , other : & QueueInner < T , S2 >) -> bool { self . len () == other . len () && self . iter () . zip (other . iter ()) . all (| (v1 , v2) | v1 == v2) } }
    };
}

impl_460!()