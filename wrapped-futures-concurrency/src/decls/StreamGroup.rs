macro_rules! deps {
    () => {
        StreamExt!();
        WakerVec!();
        PollVec!();
    };
}

macro_rules! StreamGroup {
    () => {
        deps!();
        # [doc = " A growable group of streams which act as a single unit."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " **Basic example**"] # [doc = ""] # [doc = " ```rust"] # [doc = " use futures_concurrency::stream::StreamGroup;"] # [doc = " use futures_lite::{stream, StreamExt};"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let mut group = StreamGroup::new();"] # [doc = " group.insert(stream::once(2));"] # [doc = " group.insert(stream::once(4));"] # [doc = ""] # [doc = " let mut out = 0;"] # [doc = " while let Some(num) = group.next().await {"] # [doc = "     out += num;"] # [doc = " }"] # [doc = " assert_eq!(out, 6);"] # [doc = " # });"] # [doc = " ```"] # [doc = ""] # [doc = " **Update the group on every iteration**"] # [doc = ""] # [doc = " ```rust"] # [doc = " use futures_concurrency::stream::StreamGroup;"] # [doc = " use lending_stream::prelude::*;"] # [doc = " use futures_lite::stream;"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let mut group = StreamGroup::new();"] # [doc = " group.insert(stream::once(4));"] # [doc = ""] # [doc = " let mut index = 3;"] # [doc = " let mut out = 0;"] # [doc = " let mut group = group.lend_mut();"] # [doc = " while let Some((group, num)) = group.next().await {"] # [doc = "     if index != 0 {"] # [doc = "         group.insert(stream::once(index));"] # [doc = "         index -= 1;"] # [doc = "     }"] # [doc = "     out += num;"] # [doc = " }"] # [doc = " assert_eq!(out, 10);"] # [doc = " # });"] # [doc = " ```"] # [must_use = "`StreamGroup` does nothing if not iterated over"] # [derive (Default)] # [pin_project :: pin_project] pub struct StreamGroup < S > { # [pin] streams : Slab < S > , wakers : WakerVec , states : PollVec , keys : BTreeSet < usize > , key_removal_queue : SmallVec < usize , 10 > , capacity : usize , }
    };
}

StreamGroup!()