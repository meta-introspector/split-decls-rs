macro_rules! deps {
    () => {
        CopyOptions!();
        Result!();
        TransitProcess!();
    };
}

macro_rules! dir {
    () => {
        deps!();
        # [doc = " This module includes additional methods for working with directories."] # [doc = ""] # [doc = " One of the additional features is information"] # [doc = " about process and recursion operations."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " use std::path::Path;"] # [doc = " use std::{thread, time};"] # [doc = " use std::sync::mpsc::{self, TryRecvError};"] # [doc = ""] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::dir::*;"] # [doc = " use fs_extra::error::*;"] # [doc = ""] # [doc = " fn example_copy() -> Result<()> {"] # [doc = ""] # [doc = "     let path_from = Path::new(\"./temp\");"] # [doc = "     let path_to = path_from.join(\"out\");"] # [doc = "     let test_folder = path_from.join(\"test_folder\");"] # [doc = "     let dir = test_folder.join(\"dir\");"] # [doc = "     let sub = dir.join(\"sub\");"] # [doc = "     let file1 = dir.join(\"file1.txt\");"] # [doc = "     let file2 = sub.join(\"file2.txt\");"] # [doc = ""] # [doc = "     create_all(&sub, true)?;"] # [doc = "     create_all(&path_to, true)?;"] # [doc = "     fs_extra::file::write_all(&file1, \"content1\")?;"] # [doc = "     fs_extra::file::write_all(&file2, \"content2\")?;"] # [doc = ""] # [doc = "     assert!(dir.exists());"] # [doc = "     assert!(sub.exists());"] # [doc = "     assert!(file1.exists());"] # [doc = "     assert!(file2.exists());"] # [doc = ""] # [doc = ""] # [doc = "     let options = CopyOptions {"] # [doc = "         buffer_size: 1,"] # [doc = "         ..Default::default(),"] # [doc = "     };"] # [doc = "     let (tx, rx) = mpsc::channel();"] # [doc = "     thread::spawn(move || {"] # [doc = "         let handler = |process_info: TransitProcess| {"] # [doc = "             tx.send(process_info).unwrap();"] # [doc = "             thread::sleep(time::Duration::from_millis(500));"] # [doc = "         };"] # [doc = "         copy_with_progress(&test_folder, &path_to, &options, handler).unwrap();"] # [doc = "     });"] # [doc = ""] # [doc = "     loop {"] # [doc = "         match rx.try_recv() {"] # [doc = "             Ok(process_info) => {"] # [doc = "                 println!(\"{} of {} bytes\","] # [doc = "                          process_info.copied_bytes,"] # [doc = "                          process_info.total_bytes);"] # [doc = "             }"] # [doc = "             Err(TryRecvError::Disconnected) => {"] # [doc = "                 println!(\"finished\");"] # [doc = "                 break;"] # [doc = "             }"] # [doc = "             Err(TryRecvError::Empty) => {}"] # [doc = "         }"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = ""] # [doc = " }"] # [doc = " fn main() {"] # [doc = "     example_copy();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] pub mod dir ;
    };
}

dir!();