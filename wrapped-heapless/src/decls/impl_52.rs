macro_rules! deps {
    () => {
        CapacityError!();
        Deque!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < T , const NS : usize , const ND : usize > TryFrom < [T ; NS] > for Deque < T , ND > { # [doc = " Converts a `[T; NS]` into a `Deque<T, ND>`."] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::Deque;"] # [doc = ""] # [doc = " let deq1 = Deque::<u8, 5>::try_from([1, 2, 3]).unwrap();"] # [doc = " let mut deq2 = Deque::<u8, 5>::new();"] # [doc = " deq2.push_back(1).unwrap();"] # [doc = " deq2.push_back(2).unwrap();"] # [doc = " deq2.push_back(3).unwrap();"] # [doc = ""] # [doc = " assert_eq!(deq1, deq2);"] # [doc = " ```"] type Error = (CapacityError , [T ; NS]) ; # [doc = " Converts a `[T; NS]` array into a `Deque<T, ND>`."] # [doc = ""] # [doc = " Returns back the `value` if NS > ND."] fn try_from (value : [T ; NS]) -> Result < Self , Self :: Error > { if NS > ND { return Err ((CapacityError , value)) ; } let mut deq = Self :: default () ; let value = ManuallyDrop :: new (value) ; unsafe { ptr :: copy_nonoverlapping (value . as_ptr () , deq . buffer . buffer . as_mut_ptr () . cast :: < T > () , NS ,) ; } deq . front = 0 ; deq . back = NS ; deq . full = NS == ND ; Ok (deq) } }
    };
}

impl_52!();