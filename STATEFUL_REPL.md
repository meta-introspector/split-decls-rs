# Stateful REPL - RDF Blob State System

A stateful REPL that saves its state as RDF (Resource Description Framework) blobs, allowing you to persist variables, macros, and system state between sessions.

## Features

- **Persistent State**: All variables and macros are saved as RDF triples in a local file
- **RDF Export**: Export system state in various formats (RDF/Turtle, code, docs, workspace)
- **Macro System**: Define and store custom macros that persist between sessions
- **URL Blob Format**: State is encoded as a data URL for easy transport and sharing
- **Command History**: Track all commands executed in the session

## Usage

Run the stateful REPL:

```bash
cargo run --bin stateful_repl
```

## Available Commands

- `set <var>=<value>` - Set a variable
- `get [var]` - Get variable value or list all variables
- `macro <name> <code>` - Define a macro
- `save` - Save current state to file
- `load` - Load state from file
- `export <type>` - Export state (macros|code|docs|workspace|rdf|url)
- `info` - Show state information
- `history` - Show command history
- `clear` - Clear variables and history
- `help` - Show help
- `quit/exit` - Exit REPL

## Example Session

```
🚀 Stateful REPL - RDF Blob State System
repl> set greeting=Hello World
✅ Set greeting = Hello World

repl> macro hello_world println!("Hello, World!")
✅ Added macro: hello_world

repl> info
🌐 RDF URL Blob Info:
  📅 Timestamp: 2025-12-25T17:36:52.546554821+00:00
  🎭 Macros: 1
  🔧 Capabilities: ["macros", "code", "docs", "workspace"]
  📏 Size: 719 bytes
📊 REPL Stats:
  Variables: 1
  History: 3 commands

repl> export rdf
@prefix sys: <http://split-decls.rs/system#> .
@prefix macro: <http://split-decls.rs/macro#> .
@prefix state: <http://split-decls.rs/state#> .

state:current a sys:SystemState ;
  sys:timestamp "2025-12-25T17:36:52.546560980+00:00" ;
  sys:macroCount 1 ;
  sys:capabilities ( "export" "import" "eval" "generate" ) .

macro:hello_world a sys:MacroDeclaration ;
  sys:name "hello_world" ;
  sys:type "macro" ;
  sys:sourcePath "repl:hello_world" ;
  sys:wrapper "println!(\"Hello, World!\")" ;
  sys:callable true .

repl> save
💾 State saved to repl_state.rdf

repl> quit
👋 Goodbye!
```

## State File Format

The state is saved as a base64-encoded RDF/Turtle document in `repl_state.rdf`. This format allows for:

- **Semantic representation** of system state
- **Easy parsing** by RDF tools
- **Compact encoding** for transport
- **Human-readable** when decoded

## Integration with split-decls-rs

The REPL integrates with the split-decls-rs macro system, allowing you to:

- Import macros from output2 declarations
- Define custom overlay macros
- Export state for use in other tools
- Generate code from stored macros

This creates a powerful development environment where you can iteratively build and test macro systems while maintaining persistent state across sessions.
