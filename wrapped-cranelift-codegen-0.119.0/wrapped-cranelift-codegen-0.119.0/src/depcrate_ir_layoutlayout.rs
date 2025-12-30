// Generated macro for Layout (struct)
macro_rules! Depcrate_ir_layoutLayout {
() => {
// Module: crate::ir::layout
// Provides: {"Layout"}
// Dependencies: {}
# [doc = " The `Layout` struct determines the layout of blocks and instructions in a function. It does not"] # [doc = " contain definitions of instructions or blocks, but depends on `Inst` and `Block` entity references"] # [doc = " being defined elsewhere."] # [doc = ""] # [doc = " This data structure determines:"] # [doc = ""] # [doc = " - The order of blocks in the function."] # [doc = " - Which block contains a given instruction."] # [doc = " - The order of instructions with a block."] # [doc = ""] # [doc = " While data dependencies are not recorded, instruction ordering does affect control"] # [doc = " dependencies, so part of the semantics of the program are determined by the layout."] # [doc = ""] # [derive (Debug , Clone , PartialEq , Hash)] pub struct Layout { # [doc = " Linked list nodes for the layout order of blocks Forms a doubly linked list, terminated in"] # [doc = " both ends by `None`."] blocks : SecondaryMap < Block , BlockNode > , # [doc = " Linked list nodes for the layout order of instructions. Forms a double linked list per block,"] # [doc = " terminated in both ends by `None`."] insts : SecondaryMap < Inst , InstNode > , # [doc = " First block in the layout order, or `None` when no blocks have been laid out."] first_block : Option < Block > , # [doc = " Last block in the layout order, or `None` when no blocks have been laid out."] last_block : Option < Block > , }
};
}
